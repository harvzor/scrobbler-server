SELECT d.id, d.name, dd.count, d.colour, d.deleted
FROM public.drinks d
LEFT JOIN
(
    SELECT dd.drink_id, COUNT(*) as count
    FROM public.drink_dranks dd
    GROUP BY dd.drink_id
) dd ON d.id = dd.drink_id;
