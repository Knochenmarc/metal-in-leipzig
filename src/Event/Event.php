<?php

declare(strict_types=1);

namespace MetalLE\Event;

class Event
{
    public readonly ?Image $image;

    public function __construct(
        public readonly string $name,
        public readonly \DateTimeInterface $date,
        public readonly Location $location,
        public readonly string $url,
        ?string $image = null,
        /** external id from location */
        public readonly ?string $eventId = null,
    ) {
        $this->image = $image ? new Image($image) : null;
    }

    public function getID(): string
    {
        return $this->date->format('Ymd') . $this->location->slug . $this->eventId;
    }
}