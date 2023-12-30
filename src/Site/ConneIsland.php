<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;
use Traversable;

class ConneIsland implements Site
{
    public function __construct(
        private Location $location = new Location('ci', 'Conne Island', 'https://conne-island.de'),
    ) {
    }

    public function getIterator(): Traversable
    {
        $http = new Crawler();

        $xml = $http->get('https://www.conne-island.de/rss.php?genre=Metal');
        $xml = simplexml_load_string($xml);
        foreach ($xml->channel->item as $item) {
            $name = (string) $item->title;
            yield new Event(
                         substr($name, 12),
                         new \DateTimeImmutable(substr($name, 0, 10)),
                         $this->location,
                         str_replace('http://', 'https://', (string) $item->link),
                eventId: str_replace('http://www.conne-island.de/termin/', '', (string) $item->link),
            );
        }
    }

    public function getLocations(): iterable
    {
        yield $this->location;
    }
}
