<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;

class Tankbar implements Site
{
    public function __construct(
        private Location $location = new Location('tb', 'TankBar Leipzig', 'https://tankbar-leipzig.de/',),
    ) {
    }

    public function getIterator(): \Traversable
    {
        $http      = new Crawler();
        $plainHTML = $http->get('https://tankbar-leipzig.de/tankevents/');
        if (preg_match_all(
                '#<span class="elementor-icon-list-text">(.*)</span>#siU',
                $plainHTML,
                $matches,
                PREG_SET_ORDER
            ) > 0) {
            foreach ($matches as $match) {
                [$date, $name] = explode(':', $match[1], 2);
                if (false === str_contains($name, 'Schlager')) {
                    $date = trim($date);
                    $date = substr($date, 0, -2) . '20' . substr($date, -2);
                    $name = str_replace('<br>', ' ', trim($name));

                    yield new Event(
                        html_entity_decode($name, ENT_HTML5),
                        new \DateTimeImmutable($date),
                        $this->location,
                        'https://tankbar-leipzig.de/tankevents/',
                    );
                }
            }
        }
    }

    public function getLocations(): iterable
    {
        yield $this->location;
    }
}