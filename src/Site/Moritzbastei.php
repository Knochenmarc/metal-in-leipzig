<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;
use Traversable;

class Moritzbastei implements Site
{
    use LinkedDataParser;

    public function __construct(
        private Location $location = new Location('mb', 'Moritzbastei', 'https://www.moritzbastei.de/',),
    ) {
    }

    public function getIterator(): Traversable
    {
        $http      = new Crawler();
        $plainHTML = $http->get('https://www.moritzbastei.de/programm/tag/metal');
        $plainHTML = substr($plainHTML, strpos($plainHTML, 'id="desc-tag"'));

        foreach ($this->iterateEvents($plainHTML) as $data) {
            yield new Event(
                $data->name,
                new \DateTimeImmutable($data->startDate),
                $this->location,
                $data->offers->url,
                $data->image,
            );
        }
    }

    public function getLocations(): iterable
    {
        yield $this->location;
    }
}