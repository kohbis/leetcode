# Write your MySQL query statement below
select
    t.product_id,
    t.store,
    t.price
from (
    select product_id, 'store1' as store, store1 as price from products
    union
    select product_id, 'store2' as store, store2 as price from products
    union
    select product_id, 'store3' as store, store3 as price from products
) as t
where
    t.price is not null
