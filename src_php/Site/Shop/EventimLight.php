<?php

declare(strict_types=1);

namespace MetalLE\Site\Shop;

use MetalLE\Site\Crawler;

class EventimLight implements ShopCrawler
{
    public function __construct(
        private string $venue,
        private Crawler $http = new Crawler(),
    ) {
    }

    public function fetchDates(): iterable
    {
        $plainHtml = $this->http->get('https://www.eventim-light.com/de/a/' . $this->venue,);
        if (preg_match('#window.__INITIAL_STATE__=({.*});#iU', $plainHtml, $matches)) {
            $data = json_decode($matches[1], false, 512, JSON_THROW_ON_ERROR);
            foreach ($data->events->cache as $item) {
                if ($item->category === 'Hard & Heavy') {
                    yield $item->title => new \DateTimeImmutable($item->start);
                }
            }
        }

        return [];
    }
}