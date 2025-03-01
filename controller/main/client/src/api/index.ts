const apiUrl = window.location.origin + '/api';

export async function get<T>(path: string): Promise<T> {
  const response = await fetch(`${apiUrl}${path}`);
  if (!response.ok) {
    throw new Error(`Failed to fetch ${path}`);
  }
  return await response.json();
}

export async function post<T, Body = any>(path: string, body: Body = {} as Body): Promise<T> {
  const response = await fetch(`${apiUrl}${path}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  });
  if (!response.ok) {
    throw new Error(`Failed to post to ${path}`);
  }
  return await response.json();
}

export async function put<T>(path: string, body: any): Promise<T> {
  const response = await fetch(`${apiUrl}${path}`, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  });
  if (!response.ok) {
    throw new Error(`Failed to put to ${path}`);
  }
  return await response.json();
}

export async function del<T>(path: string): Promise<T> {
  const response = await fetch(`${apiUrl}${path}`, {
    method: 'DELETE',
  });
  if (!response.ok) {
    throw new Error(`Failed to delete ${path}`);
  }
  return await response.json();
}