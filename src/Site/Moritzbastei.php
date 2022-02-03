<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;
use Traversable;

class Moritzbastei implements Site
{
    public function getIterator(): Traversable
    {
        $location = new Location(
            'mb',
            'Moritzbastei',
            'https://www.moritzbastei.de/',
            'img/moritzbastei.png',
        );

        $plainHTML = file_get_contents('https://www.moritzbastei.de/programm/tag/metal');
        if (preg_match_all(
                '#<div class="infobox.*<img src="(.*)".*<span class="date-display-single">(\d\d\.\d\d.\d\d\d\d)</span> / <span class="date-display-single">(\d\d:\d\d)</span>.*<div class="infobox-details-title">(.*)</div>.*<a href="(.*)">Mehr Infos</a>#isU',
                $plainHTML,
                $matches,
                PREG_SET_ORDER
            ) > 0) {
            foreach ($matches as $match) {
                $event = new Event(
                    $match[4],
                    new \DateTimeImmutable($match[2]. ' ' . $match[3]),
                    $location,
                    'https://www.moritzbastei.de'.$match[5],
                    $match[1],
                    $match[5],
                );

                yield $event->getID() => $event;
            }
        }
    }
}