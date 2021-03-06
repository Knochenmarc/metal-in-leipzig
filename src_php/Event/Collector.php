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

    /**
     * @param Site[] $sites
     *
     * @return Location[]
     */
    public function collectLocations(array $sites): array
    {
        $locations = [];
        foreach ($sites as $site) {
            foreach ($site->getLocations() as $location) {
                $locations[$location->slug] = $location;
            }
        }

        uasort($locations, static fn($a, $b) => strcasecmp($a->name,$b->name));

        return $locations;
    }
}