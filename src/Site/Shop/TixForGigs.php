<?php

declare(strict_types=1);

namespace MetalLE\Site\Shop;

class TixForGigs implements ShopCrawler
{
    public function __construct(
        private int $locationId,
    ) {
    }

    public function fetchDates(): iterable
    {
        $plainHtml = file_get_contents('https://www.tixforgigs.com/de-DE/Location/' . $this->locationId);
        if (preg_match('#vm.setLocationId\(\d+,({.*})\);#iU', $plainHtml, $matches)) {
            $data = json_decode($matches[1], false, 512, JSON_THROW_ON_ERROR);
            foreach ($data->futureEvents as $item) {
                if (str_contains($item->description, 'Metal')
                    || str_contains($this->crawlEvent($item->eventId)?->description ?? '', 'Metal')
                ) {
                    yield $item->title => new \DateTimeImmutable($item->beginsAt);
                }
            }
        }
    }

    private function crawlEvent(int $eventId): ?\stdClass
    {
        $plainHtml = file_get_contents('https://www.tixforgigs.com/de-DE/Event/' . $eventId);
        if (preg_match('#<script type=\'application/ld\+json\'>({.*})</script>#isU', $plainHtml, $matches)) {
            return json_decode($matches[1], false, 512, JSON_THROW_ON_ERROR);
        }

        return null;
    }
}