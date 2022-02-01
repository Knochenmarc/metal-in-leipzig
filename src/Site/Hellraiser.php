<?php

declare(strict_types=1);

namespace MetalLE\Site;

use function MetalLE\parseEvents;

class Hellraiser implements Site
{
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

    /**
     * @return Event[]
     */
    private function parseEvents(string $url): array
    {
        $events = [];
        $plainHTML = file_get_contents($url);
        if (preg_match_all(
                '#<li class="product.*<a href="(.*)".*<img.*src="(.*)".*<h2.*>(.*)<\/h2>.*<div class="date-published">(.*)<\/div>.*<\/li>#isU',
                $plainHTML,
                $matches,
                PREG_SET_ORDER
            ) > 0) {
            foreach ($matches as $match) {
                $name = str_replace(['Ticket &#8222;', '&#8220;'], '', $match[3]);
                $name = html_entity_decode($name, ENT_HTML5);
                $date = str_replace(self::DATE_SEARCH, self::DATE_REPLACE, $match[4]);

                $events[] = new Event($name, new \DateTimeImmutable($date), $match[1], $match[2]);
            }
        }

        return $events;
    }

    public function crawl(): Location
    {
        $events = array_merge(
            $this->parseEvents('https://hellraiser-leipzig.de/produkt-kategorie/tickets/'),
            $this->parseEvents('https://hellraiser-leipzig.de/produkt-kategorie/tickets/page/2/'),
            $this->parseEvents('https://hellraiser-leipzig.de/produkt-kategorie/tickets/page/3/'),
        );
        return new Location(
            'hr',
            'Hellraiser Leipzig',
            'https://hellraiser-leipzig.de/',
            'img/hellraiser.png',
            $events,
        );
    }

}