<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <title>Metal in Leipzig</title>
    <link href="bootstrap.min.css" rel="stylesheet">
    <link href="style.css" rel="stylesheet">
</head>
<body>
<div class="container-fluid">
    <div class="row title">
        <div class="col"></div>

        <?php
        /** @var \MetalLE\Site\Location[] $locations */
        foreach ($locations as $location) : ?>
            <div class="col">
                <a href="<?= $location->website ?>" target="_blank" title="<?= $location->name ?>" style="background-image: url('<?= $location->logo ?>')"></a>
            </div>
        <?php endforeach ?>
    </div>

    <?php
    /** @var \MetalLE\Site\Event[] $events */
    foreach ($chunks[0] as $events) :
    ?>
        <div class="row">
            <div class="col" x-text="new Date(Object.values(event)[0].date.date).toLocaleDateString('de-DE', {weekday: 'short', month: '2-digit', day: '2-digit' })">
                <?= format_date(reset($events)->date) ?>
            </div>
            <?php foreach ($locations as $location) : ?>
                <div class="col">
                    <?php if (isset($events[$location->slug])) : ?>
                        <a <?php if (isset($events[$location->slug]->website)) : ?> href="<?= $events[$location->slug]->website ?>"<?php endif; ?> target="_blank" >
                            <?= $events[$location->slug]->name ?>
                            <?php if (isset($events[$location->slug]->picture)) : ?>
                                <img src="<?= $events[$location->slug]->picture ?>" class="img-fluid"  alt="<?= $events[$location->slug]->name ?>"/>
                            <?php endif; ?>
                        </a>
                    <?php endif ?>
                </div>
            <?php endforeach ?>
        </div>
    <?php endforeach ?>
</div>
<footer>
    Diese Webseite dient ausschließlich meinen persönlichen Zwecken und ist somit nicht auskunftspflichtig nach <a href="https://lxgesetze.de/rstv/55" target="_blank">RStV§55</a>.<br />
    Es werden keine Cookies genutzt und es werden keine Nutzer-Daten erhoben, gespeichert oder ausgewertet.
</footer>
</body>
</html>