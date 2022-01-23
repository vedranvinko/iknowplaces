# online

Small Ruby (Sinatra) service that utilizes [Goecoder gem](https://github.com/alexreisner/geocoder) to return you some data based on your search term.

## Data

Location (struct)

```ruby
Location = Struct.new('Location', :display_name, :name, :lat, :lon)

=begin
display_name => full display name
name         => city name
lat          => latitude
lon          => longitude
=end

```

## Routes

```
GET /location/:location -> returns Location as JSON
```

### Return data

```json
{
  "display_name": "Grad Pula, Istria County, Croatia",
  "name": "Grad Pula",
  "lat": "44.8702281",
  "lon": "13.8455311"
}
```

## Usage:

### Docker

Build an image and run container:

```
docker build -t online-location .

docker run -d --name online-location-service -p 9292:9292 online-location
```
