<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;
use Traversable;

class TestData implements Site
{
    public function getIterator(): Traversable
    {
        $locations   = [];
        $locations[] = new Location(
            'aa',
            'AA AA',
            'http://example.org/aa',
        );
        $locations[] = new Location(
            'bb',
            'BBB BBB',
            'http://example.org/bbb',
        );
        $locations[] = new Location(
            'cc',
            'CCCC CCCC',
            'http://example.org/cccc',
        );

        for ($i = 0; $i < 100; $i++) {
            $now = new \DateTime();
            $now->add(new \DateInterval('P' . random_int(1, 360) . 'D'));
            yield new Event(
                $this->getRandomString(random_int(5, 100)),
                $now,
                $locations[random_int(0, 2)],
                $locations[random_int(0, 2)]->website
            );
        }
    }

    private function getRandomString($n): string
    {
        $characters   = ' &0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
        $randomString = '';

        for ($i = 0; $i < $n; $i++) {
            $index        = random_int(0, 63);
            $randomString .= $characters[$index];
        }

        return $randomString;
    }
}