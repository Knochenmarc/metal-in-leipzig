<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;

class Rocklounge implements Site
{
    public function getIterator(): \Traversable
    {
        $location = new Location(
            'rl',
            'Rocklounge Leipzig',
            'https://rocklounge-leipzig.de/',
        );

        $plainHTML = file_get_contents('https://rocklounge-leipzig.de/termine/');
        if (preg_match_all(
                '#<p>(\d\d.\d\d.\d\d) \| <a href="(.*)">(.*)</a></p>#iU',
                $plainHTML,
                $matches,
                PREG_SET_ORDER
            ) > 0) {
            foreach ($matches as $match) {
                $date  = $match[1];
                $date  = substr($date, 0, -2) . '20' . substr($date, -2);
                $event = new Event(
                    $match[3],
                    new \DateTimeImmutable($date),
                    $location,
                    $match[2],
                );

                yield $event->getID() => $event;
            }
        }
    }
}