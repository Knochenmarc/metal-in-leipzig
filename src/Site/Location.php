<?php

declare(strict_types=1);

namespace MetalLE\Site;

class Location
{
    /**
     * @param Event[] $events
     */
    public function __construct(
        public readonly string $slug,
        public readonly string $name,
        public readonly string $website,
        public readonly string $logo,
        public readonly array $events,
    ) {
    }
}