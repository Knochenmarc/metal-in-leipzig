<?php

declare(strict_types=1);

namespace MetalLE\Event;

class Location
{
    public function __construct(
        public readonly string $slug,
        public readonly string $name,
        public readonly string $website,
        public readonly string $logo,
    ) {
    }
}