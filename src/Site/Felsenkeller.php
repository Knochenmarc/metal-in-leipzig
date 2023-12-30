<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;
use Traversable;

class Felsenkeller implements Site
{
    public function getIterator(): Traversable
    {
        $location = new Location('fk', 'Felsenkeller', 'https://felsenkeller-leipzig.com');

        $html = file_get_contents('https://www.felsenkeller-leipzig.com/programm/');
        if (preg_match_all(
            '#<div class="wp-block-columns has-3-columns.*cat="Konzert".*<img src=\'(.*)\'.*<span class="date">(.*)</span>.*<p class="event-name">(.*)<span#isU',
            $html,
            $matches,
            PREG_SET_ORDER,
        )) {
            $thisYear    = (int) date('Y');
            $nextYear    = $thisYear + 1;
            $hadDecember = false;
            foreach ($matches as $key => $match) {
                $matches[$key][3] = html_entity_decode($match[3], ENT_QUOTES | ENT_SUBSTITUTE | ENT_HTML5);
                $date = substr($match[2], 0, 6);
                if (str_ends_with($date, '.12.')) {
                    $hadDecember      = true;
                    $matches[$key][2] = $date . $thisYear;
                } elseif ($hadDecember) {
                    $matches[$key][2] = $date . $nextYear;
                } else {
                    $matches[$key][2] = $date . $thisYear;
                }
            }

            yield from $this->filterByEventim($matches, $location);
            yield from $this->filterByEventimLight($matches, $location);
            // todo: reservix.de https://www.reservix.de/leipzig/venue/felsenkeller/v5341
        }
    }

    private function filterByEventim(array &$matches, Location $location): \Generator
    {
        $eventim = new Eventim();
        foreach ($eventim->fetchDates('felsenkeller-leipzig-7394') as $date) {
            foreach ($matches as $index => $match) {
                if ($date->format('d.m.Y') === $match[2]) {
                    yield new Event($match[3], $date, $location, picture: $match[1]);
                    unset($matches[$index]);
                    break;
                }
            }
        }
    }

    private function filterByEventimLight(array $matches, Location $location): \Generator
    {
        $el = new EventimLight();
        foreach ($el->fetchDates('573474f9e4b0e47b2924e6a3') as $date) {
            foreach ($matches as $index => $match) {
                if ($date->format('d.m.Y') === $match[2]) {
                    yield new Event($match[3], $date, $location, picture: $match[1]);
                    unset($matches[$index]);
                    break;
                }
            }
        }
    }
}