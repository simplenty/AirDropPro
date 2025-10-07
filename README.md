# AirDropPro

A file transfer and clipboard synchronization tool between Windows/Linux(GTK) and iOS devices implemented by Rust and Shortcuts.

## Usage

### 0. Network

- Your iPhone and computer must be on the **same LAN**, or either device must be able to connect to the **personal hotspot** of the other.
- **No cellular data** is consumed when transferring files over a hotspot.

### 1. Install AirDropPro

| OS      | How to install                                                            |
|---------|---------------------------------------------------------------------------|
| Windows | Download the portable `AirDropPlus.exe` and place it anywhere.            |
| Linux   | `sudo dpkg -i airdroppro_*.deb` – everything is configured automatically. |

### 2. Start AirDropPro

| OS      | How to start                                                                        |
|---------|-------------------------------------------------------------------------------------|
| Windows | Double-click `AirDropPlus.exe`; click **Allow** when the firewall prompt appears.   |
| Linux   | Click the *AirDropPro* icon in your application menu, or run `/usr/bin/AirDropPro`. |

### 3. Configure AirDropPro

Right-click the tray icon → **Open Config File** → edit `config.ini`.

### 4. Install the iOS Shortcut

AirDropPro is backward-compatible with the existing **AirDropPlus** shortcut.  
Until a dedicated shortcut is released, use the one below:

<https://www.icloud.com/shortcuts/d8ba54ce9e674becaf951a076ac1d967>

### 5. Configure the Shortcut

Open the shortcut and set the following fields to the values in `config.ini`:

| Field      | Meaning                                             |
|------------|-----------------------------------------------------|
| `host`     | `name` you set in PC-side settings                  |
| `key`      | Underdeveloping functions                           |
| `port`     | `port` you set in PC-side settings                  |
| `simplify` | Toggle **ON** to disable clipboard-sending from iOS |

![shortcut_conf](https://github.com/yeyt97/AirDropPlus/raw/master/pic/shortcut_conf.png)

### 6. Choose a Trigger

| Method                                   | Path                                                     |
|------------------------------------------|----------------------------------------------------------|
| Back-tap                                 | Settings → Accessibility → Touch → Back Tap → Double-Tap |
| Control Center                           | Control Center → add Shortcuts → choose **AirDrop Plus** |
| Action Button iPhone 15 Pro (and newer)  | Settings → Accessibility → Touch → Action Button         |

### 7. Remove Shortcuts’ File-Count Limit

Settings → Apps → Shortcuts → Advanced → **Allow Sharing Large Amounts of Data**  
(Required when sending multiple photos.)

### 8. Test Drive

#### Send Files
Tap the 'AirDrop Plus' shortcut from the file sharing menu.
![send_file](https://github.com/yeyt97/AirDropPlus/raw/master/pic/send_file.png)

#### Send Text
1. Copy the text.
2. Trigger the shortcut → tap **Send**.

![Send texts](https://github.com/yeyt97/AirDropPlus/raw/master/pic/shortcut_menu.png?raw=true)

#### Receive Files/Text
1. Trigger the shortcut
2. Tap the 'Receive' option to receive file or text from PC's clipboard.

![Receive files/texts](https://github.com/yeyt97/AirDropPlus/raw/master/pic/shortcut_menu.png?raw=true)

---

# API Reference

## 1. Send File
Upload a file from iOS to the PC.

**POST** `/file`  
Content-Type: `multipart/form-data`

| Field | Type | Description        |
|-------|------|--------------------|
| file  | file | The file to upload |

**Response**
```json
{
  "success": true,
  "msg": "发送成功",
  "data": null
}
```

---

## 2. Retrieve File
Download a file from the PC.

**GET** `/file/{path}`  
`{path}` = Base64-encoded **absolute** path on the PC.

**Response**  
Binary file stream.

---

## 3. Send Clipboard
Push iOS clipboard to the PC.

**POST** `/clipboard`  
Content-Type: `multipart/form-data`

| Field     | Type   | Description |
|-----------|--------|-------------|
| clipboard | string | Clipboard text |

**Response**
```json
{
  "success": true,
  "data": null
}
```

---

## 4. Get Clipboard
Fetch the current PC clipboard.

**GET** `/clipboard`

**Response**
```json
{
  "success": true,
  "data": {
    "type": "text",
    "data": "clipboard_text"
  }
}
```

```json
{
  "success": true,
  "data": {
    "type": "file",
    "data": ["file1_path_base64", "file2_path_base64"]
  }
}
```

```json
{
  "success": true,
  "data": {
    "type": "img",
    "data": "base64_encoded_image"
  }
}
```

---

## 5. Health Check
Verify the service is running.

**GET** `/`

**Response**  
Plain text: `Hello world!`