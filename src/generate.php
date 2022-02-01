<?php

declare(strict_types=1);

namespace MetalLE;

use MetalLE\Site\Bandcommunity;
use MetalLE\Site\Hellraiser;
use MetalLE\Site\Rocklounge;
use MetalLE\Site\Site;
use MetalLE\Site\Tankbar;

include "bootstrap.php";

/** @var Site[] $sites */
$sites     = [
    new Bandcommunity(),
    new Hellraiser(),
    new Rocklounge(),
    new Tankbar(),
];
$locations = $events = [];

foreach ($sites as $site) {
    $location    = $site->crawl();
    $locations[] = $location;

    foreach ($location->events as $event) {
        $events[$event->date->format('Y-m-d')][$location->slug] = $event;
    }
}

ksort($events);

$minDate = (new \DateTime())->format('Y-m-d');
$splitDate = new \DateTime();
$splitDate->add(new \DateInterval('P3M')); // next month
$chunkedDates[] = $splitDate->format('Y-m-d');
$splitDate->add(new \DateInterval('P1M')); // next month
$chunkedDates[] = $splitDate->format('Y-m-d');
$splitDate->add(new \DateInterval('P1M')); // end date
$chunkedDates[] = $splitDate->format('Y-m-d');
$chunkedDatesIterator = new \ArrayIterator($chunkedDates);

$chunks = [];
foreach ($events as $key => $event) {
    if ($key < $minDate) {
        continue;
    }
    if ($key >= $chunkedDatesIterator->current()) {
        $chunkedDatesIterator->next();
    }

    $chunks[$chunkedDatesIterator->key()][] = $event;
}

ob_start();
include 'view/index.php';
$index = ob_get_contents();
ob_end_clean();

$index = preg_replace('#\s+#', ' ', $index);
file_put_contents('/public/index.html', $index);