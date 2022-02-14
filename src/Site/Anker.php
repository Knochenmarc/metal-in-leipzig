<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;
use MetalLE\Site\Shop\Eventim;
use Traversable;

class Anker implements Site
{
    private const MONTH_SEARCH = [
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
    private const MONTH_REPLACE = ['01.', '02.', '03.', '04.', '05.', '06', '07.', '08.', '09.', '10.', '11.', '12.'];

    public function __construct(
        private Crawler $http = new Crawler(),
    ) {
    }

    public function getIterator(): Traversable
    {
        $location = new Location('ank', 'der ANKER', 'https://anker-leipzig.de');
        $eventim  = new Eventim('der-anker-leipzig-7330');

        $api = json_decode(
            $this->http->get('https://anker-leipzig.de/wp-json/wp/v2/event_listing?per_page=100'),
            false,
            512,
            JSON_THROW_ON_ERROR
        );

        $html = $this->http->get('https://anker-leipzig.de/va/veranstaltungen/');
        if (preg_match_all(
            '#wpem-single-event-widget.*<a href="(.*)".*wpem-event-date-time-text">.*,\s(.*)<#isU',
            $html,
            $matches,
            PREG_SET_ORDER,
        )) {
            foreach ($matches as $key => $match) {
                $matches[$key][2] = str_replace(self::MONTH_SEARCH, self::MONTH_REPLACE, $match[2]);
            }
        }

        foreach ($eventim->fetchDates() as $date) {
            foreach ($matches as $match) {
                if ($match[2] === $date->format('d.m.Y')) {
                    foreach ($api as $item) {
                        if ($item->link === $match[1]) {
                            yield new Event(
                                html_entity_decode($item->title->rendered),
                                $date,
                                $location,
                                $item->link,
                                $this->fetchImage($item),
                            );
                            break 2;
                        }
                    }
                }
            }
        }
    }

    private function fetchImage(\stdClass $item): ?string
    {
        if (isset($item->_links->{"wp:featuredmedia"}[0]->href)) {
            $imgApi = json_decode(
                $this->http->get($item->_links->{"wp:featuredmedia"}[0]->href),
                false,
                512,
                JSON_THROW_ON_ERROR
            );
            foreach ($imgApi->media_details->sizes as $size) {
                if ($size->width >= 300) {
                    return $size->source_url;
                }
            }
        }

        return null;
    }
}