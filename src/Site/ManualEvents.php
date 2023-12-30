<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Location;
use Traversable;

class ManualEvents implements Site
{
    public function __construct(
        private Location $messe = new Location('lm', 'Leipziger Messe', 'https://www.leipziger-messe.de'),
    ) {
    }

    public function getIterator(): Traversable
    {
//        yield new Event(
//            'Impericon Festival',
//            new \DateTimeImmutable('23.04.2022'),
//            $this->messe,
//            'https://www.impericon.com/de/festival',
//            'https://www.impericon.com/media/impericon/festivals/impericonfest_2022/20210827_imp_fest_22_vo1_microseite_bands_de.jpg',
//        );
        return new \ArrayIterator([]);
    }

    public function getLocations(): iterable
    {
//        yield $this->messe;
        return [];
    }
}