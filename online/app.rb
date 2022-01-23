# frozen_string_literal: true

require 'geocoder'
require 'json'
require 'sinatra/base'

class Request
  attr_reader :status, :params, :halt_message

  def initialize(params)
    @params = params
    validate_params
  end

  def process
    term = @params['location']
    rsp = Geocoder.search(term).first

    rsp.data
  end

  def halt?
    missing_params?
  end

  private

  def validate_params
    if missing_params?
      @status = 400
      @halt_message = "Missing :location in supplied params\n"
    else
      @status = 200
    end
  end

  def missing_params?
    missing_keys.length > 0
  end

  def missing_keys
    expected_keys = %w[location]
    params.select { |k, _| !expected_keys.include?(k) }
  end
end

Location = Struct.new('Location', :display_name, :name, :lat, :lon) do
  def to_json
    to_h.to_json
  end
end

class App < Sinatra::Base

  get '/location/:location' do
    content_type :json

    request = Request.new(params)

    halt request.status, request.halt_message if request.halt?

    obj = request.process

    location = Location.new(obj['display_name'], obj['address']['city'], obj['lat'], obj['lon'])

    location.to_json
  end
end
