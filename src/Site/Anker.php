<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;
use MetalLE\Site\Shop\Eventim;
use Traversable;

class Anker implements Site
{
    public function getIterator(): Traversable
    {
        $location = new Location('ank', 'der ANKER', 'https://anker-leipzig.de/');
        $eventim  = new Eventim('der-anker-leipzig-7330');
        foreach ($eventim->fetchDates() as $name => $date) {
            yield new Event($name, $date, $location,);
        }
    }
}