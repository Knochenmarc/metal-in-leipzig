<?php

declare(strict_types=1);

namespace MetalLE\Site\Shop;

use MetalLE\Site\Crawler;

class Eventim implements ShopCrawler
{
    public function __construct(
        private string $venue,
        private Crawler $http = new Crawler(),
    ) {
    }

    /**
     * @return iterable|\DateTimeInterface[]
     */
    public function fetchDates(): iterable
    {
        yield from $this->crawl(
            'https://www.eventim.de/city/leipzig-10/venue/' . $this->venue
            . '/?maincategoryId=1&shownonbookable=true&subcategoryId=2'
        );
    }

    private function crawl(string $url): iterable
    {
        $plainHtml = $this->http->get($url);

        if (preg_match_all(
            '#<article\s.*data-teaser-name="(.*)".*datetime="(.*)"#isU',
            $plainHtml,
            $matches,
            PREG_SET_ORDER
        )) {
            foreach ($matches as $match) {
                yield $match[1] => new \DateTimeImmutable($match[2]);
            }
        }

        if (preg_match('#<link rel="next" href="(.*)"#iU', $plainHtml, $matches)) {
            yield from $this->crawl('https://www.eventim.de' . $matches[1]);
        }
    }
}