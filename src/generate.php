<?php

declare(strict_types=1);

namespace MetalLE;

use MetalLE\Event\Collector;
use MetalLE\Event\Splitter;
use MetalLE\Site;
use MetalLE\View\Renderer;

include "bootstrap.php";

$sites  = [
//    new Site\Anker(),
//    new Site\Bandcommunity(),
    new Site\ConneIsland(),
//    new Site\HausAuensee(),
//    new Site\Hellraiser(),
//    new Site\Moritzbastei(),
//    new Site\Rocklounge(),
//    new Site\Tankbar(),
//    new Site\Werk2(),
//    new Site\TestData(),
];
$events = (new Collector())->collectEvents($sites);

if ([] === $events) {
    throw new \LogicException('no data parsed');
}

$events = (new Splitter())->splitInChunks($events);

$view      = new Renderer();
$indy      = 0;
$templates = [
    'index.php' => '/public/index.html',
    'mehr.php'  => '/public/mehr.html',
    'mehr2.php' => '/public/viel_mehr.html',
    'mehr3.php' => '/public/noch_mehr.html',
];
foreach ($templates as $template => $file) {
    $content = $view->render($template, ['events' => $events[$indy++],],);
    file_put_contents($file, preg_replace('#(\s)\s+#', '$1', $content));
}
