//! Menu system for macOS GUI applications

use crate::error::{CocoanutError, Result};
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use std::ffi::CString;

/// A macOS menu wrapper
pub struct Menu {
    ns_menu: *mut Object,
    title: String,
}

/// A macOS menu item wrapper
pub struct MenuItem {
    ns_menu_item: *mut Object,
    title: String,
    action: Option<String>,
}

impl Menu {
    /// Create a new menu
    /// 
    /// # Arguments
    /// 
    /// * `title` - The menu title
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<Menu>` containing the new menu instance
    pub fn new(title: &str) -> Result<Self> {
        unsafe {
            let menu_class = objc::class!(NSMenu);
            let ns_menu: *mut Object = msg_send![menu_class, alloc];
            
            let title_cstr = CString::new(title)
                .map_err(|e| CocoanutError::InvalidParameter(e.to_string()))?;
            
            let ns_menu: *mut Object = msg_send![
                ns_menu,
                initWithTitle: title_cstr.as_ptr()
            ];
            
            if ns_menu.is_null() {
                return Err(CocoanutError::MenuCreationFailed(
                    "Failed to create NSMenu".to_string()
                ));
            }
            
            Ok(Menu {
                ns_menu,
                title: title.to_string(),
            })
        }
    }
    
    /// Get the menu title
    pub fn title(&self) -> &str {
        &self.title
    }
    
    /// Add a menu item to this menu
    /// 
    /// # Arguments
    /// 
    /// * `item` - The menu item to add
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<()>` indicating success or failure
    pub fn add_item(&self, item: MenuItem) -> Result<()> {
        unsafe {
            let _: () = msg_send![self.ns_menu, addItem: item.ns_menu_item];
            Ok(())
        }
    }
    
    /// Get the underlying NSMenu pointer
    pub(crate) fn ns_menu(&self) -> *mut Object {
        self.ns_menu
    }
}

impl MenuItem {
    /// Create a new menu item
    /// 
    /// # Arguments
    /// 
    /// * `title` - The menu item title
    /// * `action` - Optional action selector name
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<MenuItem>` containing the new menu item instance
    pub fn new(title: &str, action: Option<&str>) -> Result<Self> {
        unsafe {
            let menu_item_class = objc::class!(NSMenuItem);
            let ns_menu_item: *mut Object = msg_send![menu_item_class, alloc];
            
            let title_cstr = CString::new(title)
                .map_err(|e| CocoanutError::InvalidParameter(e.to_string()))?;
            
            let action_sel = if action.is_some() {
                sel!(action)
            } else {
                sel!(null)
            };
            
            let empty_str = CString::new("").unwrap();
            let ns_menu_item: *mut Object = msg_send![ns_menu_item, initWithTitle:title_cstr.as_ptr() action:action_sel keyEquivalent:empty_str.as_ptr()];
            
            if ns_menu_item.is_null() {
                return Err(CocoanutError::MenuCreationFailed(
                    "Failed to create NSMenuItem".to_string()
                ));
            }
            
            Ok(MenuItem {
                ns_menu_item,
                title: title.to_string(),
                action: action.map(|s| s.to_string()),
            })
        }
    }
    
    /// Create a separator menu item
    pub fn separator() -> Result<Self> {
        unsafe {
            let menu_item_class = objc::class!(NSMenuItem);
            let ns_menu_item: *mut Object = msg_send![
                menu_item_class,
                separatorItem
            ];
            
            if ns_menu_item.is_null() {
                return Err(CocoanutError::MenuCreationFailed(
                    "Failed to create separator NSMenuItem".to_string()
                ));
            }
            
            Ok(MenuItem {
                ns_menu_item,
                title: "".to_string(),
                action: None,
            })
        }
    }
    
    /// Get the menu item title
    pub fn title(&self) -> &str {
        &self.title
    }
    
    /// Get the menu item action
    pub fn action(&self) -> Option<&str> {
        self.action.as_deref()
    }
    
    /// Set the menu item title
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        unsafe {
            let title_cstr = CString::new(title)
                .map_err(|e| CocoanutError::InvalidParameter(e.to_string()))?;
            let _: () = msg_send![self.ns_menu_item, setTitle: title_cstr.as_ptr()];
            self.title = title.to_string();
            Ok(())
        }
    }
    
    /// Get the underlying NSMenuItem pointer
    pub(crate) fn ns_menu_item(&self) -> *mut Object {
        self.ns_menu_item
    }
}

impl Drop for Menu {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.ns_menu, release];
        }
    }
}

impl Drop for MenuItem {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.ns_menu_item, release];
        }
    }
}

unsafe impl Send for Menu {}
unsafe impl Sync for Menu {}
unsafe impl Send for MenuItem {}
unsafe impl Sync for MenuItem {}
