<?php

declare(strict_types=1);

namespace MetalLE;

use MetalLE\Event\Collector;
use MetalLE\Event\Splitter;
use MetalLE\Site\Anker;
use MetalLE\Site\Bandcommunity;
use MetalLE\Site\HausAuensee;
use MetalLE\Site\Hellraiser;
use MetalLE\Site\Moritzbastei;
use MetalLE\Site\Rocklounge;
use MetalLE\Site\Tankbar;
use MetalLE\Site\TestData;
use MetalLE\Site\Werk2;
use MetalLE\View\Renderer;

include "bootstrap.php";

$sites  = [
    new Anker(),
    new Bandcommunity(),
    new HausAuensee(),
    new Hellraiser(),
    new Moritzbastei(),
    new Rocklounge(),
    new Tankbar(),
    new Werk2(),
//    new TestData(),
];
$events = (new Collector())->collectEvents($sites);

if ([] === $events) {
    throw new \LogicException('no data parsed');
}

$chunks = (new Splitter())->splitInChunks($events);

$minify = static fn($str) => preg_replace('#(\s)\s+#', '$1', $str);
$view   = new Renderer();
file_put_contents('/public/index.html', $minify($view->render('index.php', ['events' => $chunks[0]])));
file_put_contents('/public/mehr.html', $minify($view->render('mehr.php', ['events' => $chunks[1]])));
file_put_contents('/public/noch_mehr.html', $minify($view->render('mehr2.php', ['events' => $chunks[2]])));
file_put_contents('/public/viel_mehr.html', $minify($view->render('mehr3.php', ['events' => $chunks[3]])));
