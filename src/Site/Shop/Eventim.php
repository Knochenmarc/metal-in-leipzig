<?php

declare(strict_types=1);

namespace MetalLE\Site\Shop;

class Eventim implements ShopCrawler
{
    public function __construct(
        private string $venue,
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
        $customHeaders =
            "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:96.0) Gecko/20100101 Firefox/96.0\r\n"
            . "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8\r\n"
            . "Accept-Language: de,en-US;q=0.7,en;q=0.3\r\n"
            . "Accept-Encoding: identity\r\n"
            . "DNT: 1\r\n"
            . "Connection: close\r\n"
            . "Pragma: no-cache\r\n"
            . "Cache-Control: no-cache\r\n";
        $context       = stream_context_create(['http' => ['method' => 'GET', 'header' => $customHeaders]]);

        $plainHtml = file_get_contents($url, false, $context);

        if (preg_match_all(
            '#<script type="application/ld\+json">(.*"@type":"MusicEvent".*)</script>#iU',
            $plainHtml,
            $matches,
            PREG_SET_ORDER
        )) {
            foreach ($matches as $match) {
                $data = json_decode($match[1], true, 512, JSON_THROW_ON_ERROR);
                yield $data['name'] => new \DateTimeImmutable($data['startDate']);
            }
        }

        if (preg_match('#<link rel="next" href="(.*)"#iU', $plainHtml, $matches)) {
            yield from $this->crawl('https://www.eventim.de' . $matches[1]);
        }
    }
}