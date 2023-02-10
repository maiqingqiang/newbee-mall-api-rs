use crate::app::mall::UserAddresseUpdateRequest;
use crate::bootstrap::database::PooledConn;
use crate::bootstrap::result;
use crate::models::user_address::{NewUserAddress, UserAddress};

pub fn list(conn: &mut PooledConn, user_id: i64) -> result::Result<Vec<UserAddress>> {
    Ok(UserAddress::list(conn, user_id)?)
}

pub fn save(conn: &mut PooledConn, user_address: NewUserAddress) -> result::Result<usize> {
    if user_address.default_flag == UserAddress::DEFAULTED {
        UserAddress::update_default_flag(conn, user_address.user_id, UserAddress::NOT_DEFAULT)?;
    }

    Ok(UserAddress::create(conn, user_address)?)
}

pub fn update(
    conn: &mut PooledConn,
    user_id: i64,
    update_user_address: UserAddresseUpdateRequest,
) -> result::Result<usize> {
    let address_id = update_user_address.address_id.parse::<i64>()?;

    let mut user_address = UserAddress::find(conn, address_id)?;

    if user_address.default_flag == UserAddress::DEFAULTED {
        UserAddress::update_default_flag(conn, user_id, UserAddress::NOT_DEFAULT)?;
    }

    user_address.city_name = update_user_address.city_name;
    user_address.default_flag = update_user_address.default_flag;
    user_address.detail_address = update_user_address.detail_address;
    user_address.province_name = update_user_address.province_name;
    user_address.region_name = update_user_address.region_name;
    user_address.user_name = update_user_address.user_name;
    user_address.user_phone = update_user_address.user_phone;

    Ok(UserAddress::update(conn, user_address)?)
}

pub fn find(conn: &mut PooledConn, address_id: i64) -> result::Result<UserAddress> {
    Ok(UserAddress::find(conn, address_id)?)
}

pub fn delete(conn: &mut PooledConn, address_id: i64) -> result::Result<usize> {
    Ok(UserAddress::delete_by_soft(conn, address_id)?)
}

pub fn find_default(conn: &mut PooledConn, user_id: i64) -> result::Result<UserAddress> {
    Ok(UserAddress::find_default(conn, user_id)?)
}
