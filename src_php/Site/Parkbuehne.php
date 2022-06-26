<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;
use MetalLE\Site\Shop\Mawi;
use Traversable;

class Parkbuehne implements Site
{
    public function __construct(
        private Location $location = new Location('pb', 'Parkbühne im Clara-Zetkin-Park', 'https://www.parkbuehne-leipzig.com'),
    ) {
    }

    public function getIterator(): Traversable
    {
        $http = new Crawler();
        $html = $http->get('https://www.parkbuehne-leipzig.com/wordpress/veranstaltungen/');
        if (preg_match_all(
            '#<article\s.*<img\s.*src="(.*)".*<h3\s.*<a href="(.*)">(.*)</a></h3>.*</article>#isU',
            $html,
            $matches,
            PREG_SET_ORDER
        )) {
            foreach ($matches as $key => $match) {
                if (preg_match('#, (\d\d? [a-z]{2,3} \d\d\d\d)#iU', $match[0], $dateMatch)) {
                    $matches[$key][4] = new \DateTimeImmutable($dateMatch[1]);
                } else {
                    unset($matches[$key]);
                }
            }

            $mawi = new Mawi('Parkbühne Clara-Zetkin-Park');
            foreach ($mawi->fetchDates() as $date) {
                foreach ($matches as $match) {
                    if ($date->format('Ymd') === $match[4]->format('Ymd')) {
                        yield new Event(
                            html_entity_decode($match[3]),
                            $date,
                            $this->location,
                            $match[2],
                            $match[1],
                        );
                        break;
                    }
                }
            }
        }
    }

    public function getLocations(): iterable
    {
        yield $this->location;
    }
}