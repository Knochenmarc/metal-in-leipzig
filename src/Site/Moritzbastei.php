<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Location;
use Traversable;

class Moritzbastei implements Site
{
    use LinkedDataParser;

    public function getIterator(): Traversable
    {
        $location = new Location(
            'mb',
            'Moritzbastei',
            'https://www.moritzbastei.de/',
        );

        $http      = new Crawler();
        $plainHTML = $http->get('https://www.moritzbastei.de/programm/tag/metal');
        $plainHTML = substr($plainHTML, strpos($plainHTML, 'id="desc-tag"'));

        yield from $this->iterateEvents($plainHTML, $location);
    }
}