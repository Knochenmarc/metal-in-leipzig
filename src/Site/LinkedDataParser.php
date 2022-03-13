<?php

declare(strict_types=1);

namespace MetalLE\Site;

trait LinkedDataParser
{
    private function iterateEvents(string $html): iterable
    {
        if (preg_match_all('#<script type=["\']application/ld\+json["\']>(.*)</script>#isU', $html, $matches, PREG_SET_ORDER)) {
            foreach ($matches as $match) {
                $json = str_replace("\t", " ", $match[1]);
                $events = json_decode($json, false, 512, JSON_THROW_ON_ERROR);
                if (false === is_array($events)) {
                    $events = [$events];
                }
                foreach ($events as $event) {
                    if ($event->{"@type"} === 'Festival' || str_contains($event->{"@type"}, 'Event')) {
                        yield $event;
                    }
                }
            }
        }
    }
}