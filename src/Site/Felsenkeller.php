<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;
use MetalLE\Site\Shop\Eventim;
use MetalLE\Site\Shop\EventimLight;
use MetalLE\Site\Shop\ShopCrawler;
use MetalLE\Site\Shop\TixForGigs;
use Traversable;

class Felsenkeller implements Site
{
    public function getIterator(): Traversable
    {
        $location = new Location('fk', 'Felsenkeller', 'https://felsenkeller-leipzig.com');

        $html = file_get_contents('https://www.felsenkeller-leipzig.com/programm/');
        if (preg_match_all(
            '#<div class="wp-block-columns has-3-columns.*data-cat="\D*".*<img src=\'(.*)\'.*<span class="date">(.*)</span>.*<p class="event-name">(.*)</?span.*class="event-details">(.*)data-url#isU',
            $html,
            $matches,
            PREG_SET_ORDER,
        )) {
            $thisYear    = (int) date('Y');
            $nextYear    = $thisYear + 1;
            $hadDecember = false;
            foreach ($matches as $key => $match) {
                $matches[$key][3] = trim(html_entity_decode($match[3], ENT_QUOTES | ENT_SUBSTITUTE | ENT_HTML5));
                $date             = substr($match[2], 0, 6);
                if (str_ends_with($date, '.12.')) {
                    $hadDecember      = true;
                    $matches[$key][2] = $date . $thisYear;
                } elseif ($hadDecember) {
                    $matches[$key][2] = $date . $nextYear;
                } else {
                    $matches[$key][2] = $date . $thisYear;
                }
            }

            yield from $this->filterByShop(new Eventim('felsenkeller-leipzig-7394'), $matches, $location);
            yield from $this->filterByShop(new EventimLight('573474f9e4b0e47b2924e6a3'), $matches, $location, true);
            yield from $this->filterByShop(new TixForGigs(2628), $matches, $location);

            yield from $this->filterByDescription($matches, $location);
        }
    }

    private function filterByShop(
        ShopCrawler $shopCrawler,
        array &$matches,
        Location $location,
        bool $noBallroom = false
    ): \Generator {
        foreach ($shopCrawler->fetchDates() as $title => $date) {
            foreach ($matches as $index => $match) {
                if (($noBallroom === false
                        || ($noBallroom && false === str_contains($match[0], 'data-loc="Ballsaal"')))
                    && $date->format('d.m.Y') === $match[2]
                ) {
                    yield new Event($match[3], $date, $location, picture: $match[1], eventId: $match[3]);
                    unset($matches[$index]);
                    continue 2;
                }
            }
            //TODO :(
            var_dump($title, $date);
        }
    }

    private function filterByDescription(array &$matches, Location $location): \Generator
    {
        foreach ($matches as $match) {
            if (str_contains($match[4], 'Metal')
                && false === str_contains($match[3], 'Soul Tour')
            ) {
                yield new Event(
                             $match[3],
                             new \DateTimeImmutable($match[2]),
                             $location,
                    picture: $match[1],
                    eventId: $match[3]
                );
            }
        }
    }
}