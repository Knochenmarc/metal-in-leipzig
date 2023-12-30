<main>
    <?php $imgCount = 0;
    foreach ($data['events'] ?? [] as $events) : ?>
        <div class="event-date"><?= $view->format_date(reset($events)->date) ?></div>
        <div class="events">
            <?php foreach ($events as $event) : ?>
                <a <?php if ($event->url) : ?> href="<?= $event->url ?>"<?php endif; ?>
                        title="<?= htmlspecialchars($event->name) ?>"
                        target="_blank" class="event <?= $event->location->slug ?>">
                    <object><a class="event-location" href="<?= $event->location->website ?>" target="_blank"
                               title="<?= $event->location->name ?>"></a></object>
                    <?= htmlentities($event->name) ?>
                    <?php if ($event->image) : ?>
                        <picture class="event-img">
                            <source srcset="<?= $event->image->publicAvifUrl ?>" type="image/avif">
                            <img src="<?= $event->image->publicJpgUrl ?>"
                                 width="290" height="<?= (int) (290 / $event->image->ratio) ?>"
                                 <?php if ($imgCount++ > 5) : ?>loading="lazy"<?php endif; ?> alt=""/>
                        </picture>
                    <?php endif; ?>
                </a>
            <?php endforeach ?>
        </div>
    <?php endforeach ?>
</main>