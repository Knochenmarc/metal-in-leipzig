<?php

declare(strict_types=1);

namespace MetalLE;

use MetalLE\Event\Collector;
use MetalLE\Event\Splitter;
use MetalLE\Image\Optimizer;
use MetalLE\Site;
use MetalLE\View\Renderer;

include "bootstrap.php";

$sites = [
//    new Site\ManualEvents(),
//    new Site\Anker(),
//    new Site\Arena(),
//    new Site\Bandcommunity(),
//    new Site\ConneIsland(),
//    new Site\Darkflower(),
//    new Site\Felsenkeller(),
//    new Site\HausAuensee(),
//    new Site\Hellraiser(),
//    new Site\Moritzbastei(),
//    new Site\Parkbuehne(),
//    new Site\Rocklounge(),
    new Site\Taeubchenthal(),
//    new Site\Tankbar(),
//    new Site\Werk2(),
//    new Site\TestData(),
];
$collector = new Collector();
$locations = $collector->collectLocations($sites);
$events = $collector->collectEvents($sites);

if ([] === $events) {
    throw new \LogicException('no data parsed');
}

$optimizer = new Optimizer();
foreach ($events as $event) {
    if ($event->image) {
        $optimizer->optimize($event->image);
    }
}

$events = (new Splitter())->splitInChunks($events);

$view = new Renderer();
$view->render(
    'index.phtml',
    '/public/index.html',
    'Events',
    ['events' => $events[0] ?? [], 'locations' => $locations],
);
$view->render(
    'mehr.phtml',
    '/public/mehr.html',
    'Mehr Events',
    ['events' => $events[1] ?? [], 'locations' => $locations],
);
$view->render(
    'mehr2.phtml',
    '/public/viel_mehr.html',
    'Viel mehr Events',
    ['events' => $events[2] ?? [], 'locations' => $locations],
);
$view->render(
    'mehr3.phtml',
    '/public/noch_mehr.html',
    'Noch mehr Events',
    ['events' => $events[3] ?? [], 'locations' => $locations],
);
$view->render('recht.phtml', '/public/recht.html', 'Rechtliches');
