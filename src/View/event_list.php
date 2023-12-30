<main>
    <?php foreach ($data['events'] ?? [] as $events) : ?>
        <div class="event-date">
            <?= $view->format_date(reset($events)->date) ?>
        </div>
        <div class="events">
            <?php $imgCount = 0;
                foreach ($events as $event) : ?>
                <a <?php if ($event->url) : ?> href="<?= $event->url ?>"<?php endif; ?>
                    title="<?= htmlspecialchars($event->name) ?>"
                    target="_blank" class="event <?= $event->location->slug ?>">
                    <object><a class="event-location" href="<?= $event->location->website ?>" target="_blank"
                               title="<?= $event->location->name ?>"></a></object>
                    <?= htmlentities($event->name) ?>
                    <?php if ($event->picture) : ?>
                        <img src="<?= $event->picture ?>" class="event-img" <?php if ($imgCount++ > 5) : ?>loading="lazy"<?php endif; ?>
                             alt="<?= htmlspecialchars($event->name) ?>"/>
                    <?php endif; ?>
                </a>

            <?php endforeach ?>
        </div>
    <?php endforeach ?>
</main>