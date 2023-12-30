<?php

declare(strict_types=1);

namespace MetalLE;

use MetalLE\Event\Splitter;
use MetalLE\Site\Bandcommunity;
use MetalLE\Site\Hellraiser;
use MetalLE\Site\Moritzbastei;
use MetalLE\Site\Rocklounge;
use MetalLE\Site\Tankbar;
use MetalLE\Site\TestData;
use MetalLE\Site\Werk2;

include "bootstrap.php";

$sites  = [
    new Bandcommunity(),
    new Hellraiser(),
    new Moritzbastei(),
    new Rocklounge(),
    new Tankbar(),
    new Werk2(),
//    new TestData(),
];
$events = [];
foreach ($sites as $site) {
    $events += iterator_to_array($site);
}

if([] === $events) {
    throw new \LogicException('no data parsed');
}

$chunks = (new Splitter())->splitInChunks($events);

ob_start();
include 'view/index.php';
$index = ob_get_clean();

$index = preg_replace('#(\s)\s+#', '$1', $index);
file_put_contents('/public/index.html', $index);