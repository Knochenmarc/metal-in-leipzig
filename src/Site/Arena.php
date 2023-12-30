<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;
use MetalLE\Site\Shop\Eventim;
use Traversable;

class Arena implements Site
{
    private const URL = 'https://www.quarterback-immobilien-arena.de';

    public function __construct(
        private Crawler $http = new Crawler(),
    ) {
    }

    public function getIterator(): Traversable
    {
        $rb = new Location('rb', 'Red Bull Arena', self::URL . '/red-bull-arena');
        yield from $this->crawlContent(2, $rb, new Eventim('red-bull-arena-16304'));

        $immo = new Location('qi', 'QUARTERBACK Immobilien ARENA', self::URL . '/quarterback-immobilien-arena');
        yield from $this->crawlContent(1, $immo, new Eventim('quarterback-immobilien-arena-leipzig-383'));

        $fw = new Location('fw', 'Festwiese Leipzig', self::URL . '/festwiese-leipzig');
        yield from $this->crawlContent(3, $fw, new Eventim('festwiese-leipzig-7410'));
    }

    private function crawlContent(int $locationId, Location $location, Eventim $eventim, int $page = 1): \Generator
    {
        $html = $this->http->get(
            self::URL . '/events-tickets/events?tx_ifabeventmanagementextend_searchfilter[eventSearch][category]=1'
            . '&tx_ifabeventmanagementextend_searchfilter[eventSearch][location]=' . $locationId
            . '&tx_ifabeventmanagement_events[@widget_0][currentPage]=' . $page
        );

        if (preg_match_all(
            '#<div class="event".*<a href="(.*)">.*<source srcset="(.*)" media="\(max-width: 320px\)">.*<div>\w+,\s+(\d\d\.\d\d\.\d\d\d\d)</div>.*<h2>(.*)</h2>#isU',
            $html,
            $matches,
            PREG_SET_ORDER
        )) {
            foreach ($eventim->fetchDates() as $date) {
                foreach ($matches as $match) {
                    if ($date->format('d.m.Y') === $match[3]) {
                        yield new Event(html_entity_decode($match[4]), $date, $location, self::URL . $match[1], self::URL . $match[2]);
                        break;
                    }
                }
            }
        }

        if (str_contains($html, '<li class="next">')) {
            yield from $this->crawlContent($locationId, $location, $eventim, $page + 1);
        }
    }
}