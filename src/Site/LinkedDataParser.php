<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;

trait LinkedDataParser
{
    /**
     * @return iterable|Event[]
     * @throws \JsonException
     */
    private function iterateEvents(string $html, Location $location): iterable
    {
        if (preg_match_all('#<script type="application/ld\+json">(.*)</script>#isU', $html, $matches, PREG_SET_ORDER)) {
            foreach ($matches as $match) {
                $event = json_decode($match[1], false, 512, JSON_THROW_ON_ERROR);
                yield new Event(
                    $event->name,
                    new \DateTimeImmutable($event->startDate),
                    $location,
                    $event->url ?? $event->offers->url ?? $location->website,
                    $event->image,
                );
            }
        }
    }
}