<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;
use Traversable;

class Werk2 implements Site
{
    private const URL = 'https://www.werk-2.de';

    public function getIterator(): Traversable
    {
        $location = new Location(
            'w2',
            'WERK 2',
            self::URL,
        );

        yield from $this->fetch(self::URL, $location);
        yield from $this->fetch(self::URL . '/programm/vorschau', $location);
    }

    private function fetch(string $url, Location $location): iterable
    {
        $hasDecember = false;
        $currentYear = (int) date('Y');
        $http        = new Crawler();
        $plainHTML   = $http->get($url);
        if (preg_match_all('#<li .*</li>#isU', $plainHTML, $matches)) {
            foreach ($matches[0] as $listItem) {
                if (preg_match(
                    '#<div class=\'monat\'>(.*)</div>.*<div class=\'tag\'>(\d\d)</div>.*<p class=\'typen\'>.*Metal.*</p>.*<h2><a href=\'(.*)\'>(.*)</a>.*<img .*src=\'(.*)\'#isU',
                    $listItem,
                    $liMatches
                )) {
                    $date = new \DateTime();

                    if ($hasDecember === false && $liMatches[1] === 'Dezember') {
                        $hasDecember = true;
                    }

                    if ($hasDecember && $liMatches[1] !== 'Dezember') {
                        $date->setDate($currentYear + 1, $this->translateMonth($liMatches[1]), (int) $liMatches[2]);
                    } else {
                        $date->setDate($currentYear, $this->translateMonth($liMatches[1]), (int) $liMatches[2]);
                    }


                    yield new Event(
                        $liMatches[4],
                        $date,
                        $location,
                        self::URL . $liMatches[3],
                        self::URL . str_replace('_liste.', '_detail.', $liMatches[5]),
                        $liMatches[3],
                    );
                }
            }
        }

        return [];
    }

    private function translateMonth(string $month): int
    {
        return match ($month) {
            'Januar' => 1,
            'Februar' => 2,
            'März' => 3,
            'April' => 4,
            'Mai' => 5,
            'Juni' => 6,
            'July' => 7,
            'August' => 8,
            'September' => 9,
            'Oktober' => 10,
            'November' => 11,
            'Dezember' => 12,
        };
    }
}