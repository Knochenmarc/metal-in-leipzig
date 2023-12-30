<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;

class Hellraiser implements Site
{
    private const URL = 'https://hellraiser-leipzig.de';

    private const DATE_SEARCH = [
        'Mo, ',
        'Di, ',
        'Mi, ',
        'Do, ',
        'Fr, ',
        'Sa, ',
        'So, ',
        ' Januar ',
        ' Februar ',
        ' März ',
        ' April ',
        ' Mai ',
        ' Juni ',
        ' Juli ',
        ' August ',
        ' September ',
        ' Oktober ',
        ' November ',
        ' Dezember ',
    ];
    private const DATE_REPLACE = [
        '',
        '',
        '',
        '',
        '',
        '',
        '',
        '01.',
        '02.',
        '03.',
        '04.',
        '05.',
        '06.',
        '07.',
        '08.',
        '09.',
        '10.',
        '11.',
        '12.',
    ];

    public function __construct(
        private Location $location = new Location('hr', 'Hellraiser Leipzig', self::URL,),
    ) {
    }

    public function getIterator(): \Traversable
    {
        $http = new Crawler();

        yield from $this->parseEvents($http->get(self::URL . '/produkt-kategorie/tickets/'));
        yield from $this->parseEvents($http->get(self::URL . '/produkt-kategorie/tickets/page/2/'));
        yield from $this->parseEvents($http->get(self::URL . '/produkt-kategorie/tickets/page/3/'));
    }

    /**
     * @return Event[]
     */
    private function parseEvents(string $plainHTML): iterable
    {
        if (preg_match_all(
                '#<li class="product.*<a href="(.*)".*<img.*src="(.*)".*<h2.*>(.*)</h2>.*<div class="date-published">(.*)</div>.*</li>#isU',
                $plainHTML,
                $matches,
                PREG_SET_ORDER
            ) > 0) {
            foreach ($matches as $match) {
                $name = str_replace(['Ticket &#8222;', '&#8220;'], '', $match[3]);
                $name = html_entity_decode($name, ENT_HTML5);
                $date = str_replace(self::DATE_SEARCH, self::DATE_REPLACE, $match[4]);

                yield new Event(
                    $name,
                    new \DateTimeImmutable($date),
                    $this->location,
                    $match[1],
                    $match[2],
                    str_replace(self::URL, '', $match[1])
                );
            }
        }
    }

    public function getLocations(): iterable
    {
        yield $this->location;
    }
}