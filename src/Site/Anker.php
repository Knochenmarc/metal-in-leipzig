<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;
use Traversable;

class Anker implements Site
{
    public function getIterator(): Traversable
    {
        $location = new Location('ank', 'der ANKER', 'https://anker-leipzig.de/');
        $eventim  = new Eventim();
        foreach ($eventim->fetchDates('der-anker-leipzig-7330') as $name => $date) {
            yield new Event($name, $date, $location,);
        }
    }
}