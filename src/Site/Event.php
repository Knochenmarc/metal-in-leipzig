<?php

declare(strict_types=1);

namespace MetalLE\Site;

class Event
{
    public function __construct(
        public readonly string $name,
        public readonly \DateTimeImmutable $date,
        public readonly ?string $website = null,
        public readonly ?string $picture = null,
    ) {
    }
}