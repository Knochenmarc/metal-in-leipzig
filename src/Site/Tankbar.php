<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;

class Tankbar implements Site
{
    public function getIterator(): \Traversable
    {
        $location = new Location(
            'tb',
            'TankBar Leipzig',
            'https://tankbar-leipzig.de/',
        );

        $plainHTML = file_get_contents('https://tankbar-leipzig.de/tankevents/');
        if (preg_match_all(
                '#<span class="elementor-icon-list-text">(.*)</span>#siU',
                $plainHTML,
                $matches,
                PREG_SET_ORDER
            ) > 0) {
            foreach ($matches as $match) {
                $txt   = $match[1];
                $parts = explode(' : ', $txt);
                $date  = $parts[0];
                $date  = substr($date, 0, -2) . '20' . substr($date, -2);

                yield new Event(
                    html_entity_decode($parts[1], ENT_HTML5),
                    new \DateTimeImmutable($date),
                    $location,
                );
            }
        }
    }
}