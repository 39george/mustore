--! get_creator_marks_avg
SELECT AVG(mark), COUNT(mark)
FROM service_reviews
JOIN service_orders
ON service_reviews.service_orders_id = service_orders.id
JOIN services
ON service_orders.services_id = services.id
WHERE services.creator_id = :creator_id;

-- get_creator_inbox_response_rate

