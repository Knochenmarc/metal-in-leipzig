<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;
use Traversable;

interface Site extends \IteratorAggregate
{
    /**
     * @return Event[]
     */
    public function getIterator(): Traversable;

    /**
     * @return Location[]
     */
    public function getLocations(): iterable;
}