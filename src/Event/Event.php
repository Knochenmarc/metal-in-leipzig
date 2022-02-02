<?php

declare(strict_types=1);

namespace MetalLE\Event;

class Event
{
    public function __construct(
        public readonly string $name,
        public readonly \DateTimeInterface $date,
        public readonly Location $location,
        public readonly ?string $url = null,
        public readonly ?string $picture = null,
        /** external id from location */
        public readonly ?string $eventId = null,
    ) {
    }

    public function getID(): string
    {
        return $this->date->format('Ymd') . $this->location->slug . $this->eventId;
    }
}