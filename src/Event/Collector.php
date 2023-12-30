<?php

declare(strict_types=1);

namespace MetalLE\Event;

use MetalLE\Site\Site;

class Collector
{
    /**
     * @param Site[] $sites
     *
     * @return Event[]
     */
    public function collectEvents(array $sites): array
    {
        $events = [];
        foreach ($sites as $site) {
            foreach ($site as $event) {
                $events[$event->getID()] = $event;
            }
        }

        ksort($events);

        return $events;
    }
}