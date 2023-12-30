<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;
use MetalLE\Site\Shop\Eventim;
use MetalLE\Site\Shop\LiveGigs;
use MetalLE\Site\Shop\ShopCrawler;
use Traversable;

class Taeubchenthal implements Site
{
    private const URL = 'https://www.taeubchenthal.com/';

    public function __construct(
        private Location $location = new Location('tt', 'Täubchenthal', self::URL),
    ) {
    }

    public function getIterator(): Traversable
    {
        $http = new Crawler();
        $html = $http->get(self::URL . 'programm');
        if (preg_match_all(
            '#<div class="event event--list.*<img src="(.*)".*<h2><a href="(programm/.*)".*>(.*)</a></h2>.*<time datetime="(.*)">#isU',
            $html,
            $matches,
            PREG_SET_ORDER
        )) {
            yield from $this->filter(new Eventim('taeubchenthal-leipzig-18055'), $matches);
            yield from $this->filter(new LiveGigs('Täubchenthal'), $matches);
        }
    }

    private function filter(ShopCrawler $shop, array &$matches): \Generator
    {
        foreach ($shop->fetchDates() as $date) {
            foreach ($matches as $key => $match) {
                $eventDate = new \DateTimeImmutable($match[4]);
                if ($date->format('d.m.Y') === $eventDate->format('d.m.Y')) {
                    yield new Event(
                        $match[3],
                        $eventDate,
                        $this->location,
                        self::URL . $match[2],
                        self::URL . $match[1],
                    );
                    unset($matches[$key]);
                    continue 2;
                }
            }
        }
    }

    public function getLocations(): iterable
    {
        yield $this->location;
    }
}