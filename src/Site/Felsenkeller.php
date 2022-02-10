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
            '#<div class="wp-block-columns has-3-columns.*data-cat="\D*".*<span class="date">(.*)</span>.*<p class="event-name">(.*)</?span.*class="event-details">(.*)data-url#isU',
            $html,
            $matches,
            PREG_SET_ORDER,
        )) {
            $thisYear    = (int) date('Y');
            $nextYear    = $thisYear + 1;
            $hadDecember = false;
            foreach ($matches as $key => $match) {
                $matches[$key][2] = trim(html_entity_decode($match[2], ENT_QUOTES | ENT_SUBSTITUTE | ENT_HTML5));
                $date             = substr($match[1], 0, 6);
                if (str_ends_with($date, '.12.')) {
                    $hadDecember      = true;
                    $matches[$key][1] = $date . $thisYear;
                } elseif ($hadDecember) {
                    $matches[$key][1] = $date . $nextYear;
                } else {
                    $matches[$key][1] = $date . $thisYear;
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
                    && $date->format('d.m.Y') === $match[1]
                ) {
                    yield new Event(
                                 $match[2],
                                 $date,
                                 $location,
                        picture: $this->parsePicture($match[0]),
                        eventId: $match[2]
                    );
                    unset($matches[$index]);
                    continue 2;
                }
            }
        }
    }

    private function filterByDescription(array &$matches, Location $location): \Generator
    {
        foreach ($matches as $match) {
            if (str_contains($match[3], 'Metal')
                && false === str_contains($match[2], 'Soul Tour')
            ) {
                yield new Event(
                             $match[2],
                             new \DateTimeImmutable($match[1]),
                             $location,
                    picture: $this->parsePicture($match[0]),
                    eventId: $match[2]
                );
            }
        }
    }

    private function parsePicture(string $html): ?string
    {
        if (preg_match("#<img src='(.*)'#iU", $html, $match)) {
            return $match[1];
        }

        return null;
    }
}