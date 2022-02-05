<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <title>Metal in Leipzig</title>
    <link href="style.css" rel="stylesheet">
</head>
<body>
<header>
    <div><span class="update-time"><?= date('d.m. H:i') ?> zuletzt aktualisiert</span></div>
    <div><a href="" class="logo"></a></div>
    <div>
        <a href="https://github.com/Knochenmarc/metal-in-leipzig" target="_blank" title="Fork me on Github"
           class="github"></a>
    </div>
</header>
<main>
    <?php foreach ($chunks[0] as $events) : ?>
        <div class="event-date">
            <?= format_date(reset($events)->date) ?>
        </div>
        <div class="events">
            <?php foreach ($events as $event) : ?>
                <a <?php if ($event->url) : ?> href="<?= $event->url ?>"<?php endif; ?>
                        title="<?= htmlspecialchars($event->name) ?>"
                        target="_blank" class="event <?= $event->location->slug ?>">
                    <object><a class="event-location" href="<?= $event->location->website ?>" target="_blank"
                       title="<?= $event->location->name ?>"></a></object>
                    <?= htmlentities($event->name) ?>
                    <?php if ($event->picture) : ?>
                        <img src="<?= $event->picture ?>" class="event-img" loading="lazy"
                             alt="<?= htmlspecialchars($event->name) ?>"/>
                    <?php endif; ?>
                </a>

            <?php endforeach ?>
        </div>
    <?php endforeach ?>
</main>
<footer>
    Diese Webseite dient ausschließlich meinen persönlichen Zwecken und ist somit nicht auskunftspflichtig nach <a
            href="https://lxgesetze.de/rstv/55" target="_blank">RStV§55</a>.<br/>
    Es werden keine Cookies genutzt und es werden keine Nutzer-Daten erhoben, gespeichert oder ausgewertet.
</footer>
</body>
</html>