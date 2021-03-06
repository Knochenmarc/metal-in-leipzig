<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;

class Bandcommunity implements Site
{
    public function __construct(
        private Location $location = new Location('bc', 'Bandcommunity Leipzig', 'https://bandcommunity-leipzig.org/'),
    ) {
    }

    public function getIterator(): \Traversable
    {
        $http = new Crawler();

        $plainHTML = $http->get('https://bandcommunity-leipzig.org/blog.html');
        if (preg_match_all(
                '#<div class="event layout_upcoming upcoming.*<span class="date">(.*)</span>.*<a href="(.*)" title="(.*)">#isU',
                $plainHTML,
                $matches,
                PREG_SET_ORDER
            ) > 0
        ) {
            foreach ($matches as $match) {
                $image     = null;
                $url       = 'https://bandcommunity-leipzig.org/' . $match[2];
                $eventPage = $http->get($url);
                if (
                    preg_match('#<div class="image"><img src="(.*)"#iU', $eventPage, $eventMatch)
                    && false === empty($eventMatch[1])
                ) {
                    $image = 'https://bandcommunity-leipzig.org/' . $eventMatch[1];
                }

                $date = $match[1];
                if (strlen($date) < 7) {
                    $date .= date('Y');
                }

                $name = preg_replace('#\s+\(.*\)\s*$#U', '', $match[3]);
                $name = html_entity_decode($name);

                yield new Event(
                    $name,
                    new \DateTimeImmutable($date),
                    $this->location,
                    $url,
                    $image,
                );
            }
        }
    }

    public function getLocations(): iterable
    {
        yield $this->location;
    }
}