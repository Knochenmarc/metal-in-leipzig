<?php

declare(strict_types=1);

namespace MetalLE;

use MetalLE\Event\Splitter;
use MetalLE\Site\Bandcommunity;
use MetalLE\Site\Hellraiser;
use MetalLE\Site\Rocklounge;
use MetalLE\Site\Tankbar;
use MetalLE\Site\TestData;

include "bootstrap.php";

$sites  = [
    new Bandcommunity(),
    new Hellraiser(),
    new Rocklounge(),
    new Tankbar(),
//    new TestData(),
];
$events = [];
foreach ($sites as $site) {
    $events += iterator_to_array($site);
}

$chunks = (new Splitter())->splitInChunks($events);

ob_start();
include 'view/index.php';
$index = ob_get_contents();
ob_end_clean();

$index = preg_replace('#\s+#', ' ', $index);
file_put_contents('/public/index.html', $index);