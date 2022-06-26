<?php

declare(strict_types=1);

namespace MetalLE\Site\Shop;

use MetalLE\Site\Crawler;
use MetalLE\Site\LinkedDataParser;

class LiveGigs implements ShopCrawler
{
    use LinkedDataParser;

    public function __construct(
        private string $location,
    ) {
    }

    public function fetchDates(): iterable
    {
        $http = new Crawler();
        $html = $http->get('https://www.livegigs.de/leipzig/metal');

        foreach ($this->iterateEvents($html) as $event) {
            if ($event->location->name === $this->location) {
                yield $event->name => new \DateTimeImmutable($event->startDate);
            }
        }
    }
}