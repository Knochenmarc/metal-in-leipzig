<?php

declare(strict_types=1);

namespace MetalLE\Event;

class Splitter
{
    /**
     * @param Event[] $events
     *
     * @return Event[][][]
     */
    public function splitInChunks(array $events): array
    {
        ksort($events);

        $interval = new \DateInterval('P3M'); // 3 months

        $minDate   = (new \DateTime())->format('Ymd');
        $splitDate = new \DateTime();
        for ($i=0; $i<4; $i++) {
            $splitDate->add($interval);
            $chunkedDates[] = $splitDate->format('Ymd');
        }

        $chunkedDatesIterator = new \ArrayIterator($chunkedDates);

        $chunks = [];
        foreach ($events as $key => $event) {
            if ($key < $minDate) {
                continue;
            }
            if ($key >= $chunkedDatesIterator->current()) {
                $chunkedDatesIterator->next();
            }

            $chunks[$chunkedDatesIterator->key()][$event->date->format('Ymd')][$key] = $event;
        }

        return $chunks;
    }
}